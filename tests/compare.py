#!/usr/bin/env python

import sys
import os.path
import argparse

TESTWIDTH = 20
TESTHEIGHT = 10


desc = '''
Determine equality in test results.

Each test includes an expected result and a main program.  The main
program entails constructing an ncurses window with specific
dimensions and dumping the contents to an outfile.  This program
compares the outfile to the expected file.  If they are the same then
the test concludes successfully.
'''

parser = argparse.ArgumentParser(description=desc)

parser.add_argument('testpath', metavar='p', type=str, 
                    help='the path of a test directory')

args = parser.parse_args()

def E(err, msg): return err + '\n' + msg

def err(s):
    sys.stderr.write(s+"\n")

def die(s):
    err(s)
    sys.exit(1)

def verifyFileExists(p):
    if not os.path.isfile(p):
        die("could not find file: " + p)

def verifyDimension(xs):
    if len(xs) != TESTHEIGHT:
        return "Test files must have %d lines" % TESTHEIGHT
    for line in xs:
        if len(line) != TESTWIDTH:
            return "Test file lines must have with of %d lines" % TESTWIDTH
    return None

def boxOutput(s):
    #          +---+
    #  abc ->  |abc|
    #  def ->  |def|
    #          +---+
    lines = [x for x in s.split("\n") if x != '']
    e = verifyDimension(lines)
    if e: return "", e

    linelen = len(lines[0])
    cap = '+' + linelen*'-' + '+'

    box = [cap]
    for line in lines:
        box.append('|' + line + '|')
    box.append(cap + '\n') # extra '\n' to match window::screenDump

    return '\n'.join(box), None

def sideBySide(exp, out):
    boxe, e = boxOutput(exp)
    if e: return E(e, "Couldn't make sideBySide")
    boxo, e = boxOutput(out)
    if e: return E(e, "Couldn't make sideBySide")

    bes = boxe.strip().split('\n')
    bos = boxo.strip().split('\n')

    m1 = " Expected           "
    m2 = " Got Output         "
    sep = "     "
    header = m1 + sep + m2

    err(header)
    for (left, right) in zip(bes, bos):
        err(left + "   " + right)

def main():
    testname = sys.argv[1]

    exp_path = os.path.join("pass", testname, "expected")
    out_path = os.path.join("pass", testname, "output")
    
    verifyFileExists(exp_path)
    verifyFileExists(out_path)
    
    exp = open(exp_path).read()
    out = open(out_path).read()

    if exp != out:
        sideBySide(exp, out)
        err("FAIL: " + testname)
        sys.exit(1)
    else:
        err("PASS: " + testname)

if __name__ == "__main__":
    main()


