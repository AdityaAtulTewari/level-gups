#!/usr/bin/gnuplot

L(P1,P2,P3,PM) = P1 * L1 + P2 * L2 + P3 * L3 + PM * LM
L1 =   1
L2 =  10
L3 = 100
LM =1000
set dummy P1,P2,P3,PM
fit L(P1,P2,P3,PM) "/dev/stdin" using 2:3:4:5:1 via L1,L2,L3,LM

set print "| paste - -"
print "L1"
print L1
print "L2"
print L2
print "L3"
print L3
print "LM"
print LM
