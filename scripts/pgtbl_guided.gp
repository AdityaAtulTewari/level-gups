#!/usr/bin/gnuplot

iL1 =   3
iL2 =  11
iL3 =  40
iLM = 200

mL1 = 1
xL1 = 10
mL2 = 10
xL2 = 25
mL3 = 25
xL3 = 200

set dummy P1,P2,P3,PM

L(P1,P2,P3,PM) = P1 * (mL1 + (xL1 - mL1)/(1 + iL1* iL1)) + P2 * (mL2 + (xL2 - mL2)/(1 + iL2* iL2)) + P3 * (mL3 + (xL3 - mL3)/(1 + iL3* iL3)) + PM *iLM *iLM

fit L(P1,P2,P3,PM) "/dev/stdin" using 2:3:4:5:1 via iL1,iL2,iL3,iLM

L1 = mL1 + (xL1 - mL1)/(1 + iL1 * iL1)
L2 = mL2 + (xL2 - mL2)/(1 + iL2 * iL2)
L3 = mL3 + (xL3 - mL3)/(1 + iL3 * iL3)
LM = iLM * iLM

set print "| paste - -"
print "L1"
print L1
print "L2"
print L2
print "L3"
print L3
print "LM"
print LM
