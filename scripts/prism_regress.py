#!/usr/bin/python3

import argparse
import sys
import matplotlib.pyplot as plt
import numpy as np
import re
import scipy as sp
import scipy.optimize
import scipy.stats

parser = argparse.ArgumentParser()
parser.add_argument('-o', action='store', dest="ouputf", help="Ouput FileN", default="")
parser.add_argument('-i', action='store', dest="inputf", help="Input FileN", default="")

def parse_input(inputf):
  A=[]
  b=[]
  for line in inputf.readlines():
    values = []
    broken_line = re.split(r'\s+', line.rstrip())
    b.append(float(broken_line[0]))
    for value in broken_line[1:]:
      values.append(float(value))
    A.append(values)
  return A,b

#L(P1,P2,P3,PM) = P1 * L1 + P2 * L2 + P3 * L3 + PM * LM
def regression(matrix, vector):
  res = sp.optimize.lsq_linear(matrix, vector, bounds=(1,np.inf))
  if not res["success"]:
    print("Failed to converge yadumb biatch", file=sys.stderr)
    sys.exit(-1)

  unexplained_var = np.var(res["fun"])/np.var(vector)
  return res["x"], unexplained_var

def reshape(reg_matrix):
  toret = []
  for vec in reg_matrix:
    add =[]
    tot = 0
    for i in reversed(vec):
      tot += i
      add.append(tot)
    toret.append(add[::-1])
  return toret

if __name__ == "__main__":
  args = parser.parse_args()
  inputf = sys.stdin
  ouputf = sys.stdout
  if args.inputf != "" or args.inputf is None:
    inputf = open(args.inputf)
  if args.ouputf != "" or args.ouputf is None:
    ouputf = open(args.ouputf)
  reg_matrix, reg_vector = parse_input(inputf)
  res_matrix = reshape(reg_matrix)
  res, res_v = regression(res_matrix, reg_vector)
  outputNames = ["L1", "L2", "L3", "LM"]
  tot = 0
  for name, val in zip(outputNames, res):
    tot += val
    print(name + "\t" + str(tot) + "\t" + str(res_v), file=ouputf)
