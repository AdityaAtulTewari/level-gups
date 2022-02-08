#!/usr/bin/python3

import argparse
import sys
import matplotlib.pyplot as plt
import numpy as np
import re

parser = argparse.ArgumentParser()
parser.add_argument('-t', action='store', dest="gtitle", help="Graph Title", required=True)
parser.add_argument('-x', action='store', dest="xlabel", help="XAxis Label", required=True)
parser.add_argument('-y', action='store', dest="ylabel", help="YAxis Label", required=True)
parser.add_argument('-o', action='store', dest="ouputf", help="Ouput FileN", required=True)
parser.add_argument('-i', action='store', dest="inputf", help="Input FileN", default="")


def graph(gtitle, xlabel, ylabel, inputf, ouputf):
  #fig = plt.figure(figsize=(10,7))
  #ax = fig.add_axes([0,0,1,1])
  xaxis = []
  yaxis = []
  eaxis = []
  for line in inputf.readlines():
    broken_line = re.split(r'\s+', line.rstrip())
    print(broken_line)
    yaxis.append(float(broken_line[0]))
    xaxis.append(      broken_line[1])
    if len(broken_line) > 2:
      eaxis.append(float(broken_line[2]))
    else :
      eaxis.append(0)

  width = 0.25
  ind = np.arange(len(yaxis))
  plt.bar(ind, yaxis, width, yerr=eaxis, color='b')
  plt.xticks(ind, xaxis)
  plt.title(gtitle)
  plt.ylabel(ylabel)
  plt.xlabel(xlabel)
  plt.savefig(ouputf)
  #plt.show()

if __name__ == "__main__":
  args = parser.parse_args()
  inputf = sys.stdin
  if args.inputf != "" or args.inputf is None:
    inputf = open(args.inputf)
  graph(args.gtitle,args.xlabel,args.ylabel,inputf,args.ouputf)
