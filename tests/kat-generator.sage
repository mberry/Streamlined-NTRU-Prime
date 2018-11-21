p = 761; q61 = 765; q = 6*q61+1; t = 143
Zx.<x> = ZZ[]; R.<xp> = Zx.quotient(x^p-x-1)
Fq = GF(q); Fqx.<xq> = Fq[]; Rq.<xqp> = Fqx.quotient(x^p-x-1)
F3 = GF(3); F3x.<x3> = F3[]; R3.<x3p> = F3x.quotient(x^p-x-1)

import hashlib
def hash(s): h = hashlib.sha512(); h.update(s); return h.digest()

def random32(): return randrange(-2^31,2^31)
def random32even(): return random32() & (-2)
def random321mod4(): return (random32() & (-3)) | 1
def randomrange3(): return ((random32() & 0x3fffffff) * 3) >> 30

import itertools
def concat(lists): return list(itertools.chain.from_iterable(lists))

def nicelift(u):
  return lift(u + q//2) - q//2

def nicemod3(u): # r in {0,1,-1} with u-r in {...,-3,0,3,...}
  return u - 3*round(u/3)

def int2str(u,bytes):
  return ''.join(chr((u//256^i)%256) for i in range(bytes))

def str2int(s):
  return sum(ord(s[i])*256^i for i in range(len(s)))

def seq2str(u,radix,batch,bytes): # radix^batch <= 256^bytes
  return ''.join(int2str(sum(u[i+t]*radix^t for t in range(batch)),bytes)
                 for i in range(0,len(u),batch))

def str2seq(s,radix,batch,bytes):
  u = [str2int(s[i:i+bytes]) for i in range(0,len(s),bytes)]
  return concat([(u[i]//radix^j)%radix for j in range(batch)] for i in range(len(u)))

def encodeZx(m): # assumes coefficients in range {-1,0,1}
  m = [m[i]+1 for i in range(p)] + [0]*(-p % 4)
  return seq2str(m,4,4,1)

def decodeZx(mstr):
  m = str2seq(mstr,4,4,1)
  return Zx([m[i]-1 for i in range(p)])

def encodeRq(h):
  h = [q//2 + nicelift(h[i]) for i in range(p)] + [0]*(-p % 5)
  return seq2str(h,6144,5,8)[:1218]

def decodeRq(hstr):
  h = str2seq(hstr,6144,5,8)
  if max(h) >= q: raise Exception("pk out of range")
  return Rq([h[i]-q//2 for i in range(p)])

def encoderoundedRq(c):
  c = [q61 + nicelift(c[i]/3) for i in range(p)] + [0]*(-p % 6)
  return seq2str(c,1536,3,4)[:1015]

def decoderoundedRq(cstr):
  c = str2seq(cstr,1536,3,4)
  if max(c) > q61*2: raise Exception("c out of range")
  return 3*Rq([c[i]-q61 for i in range(p)])

def randomR(): # R element with 2t coeffs +-1
  L = [random32even() for i in range(2*t)]
  L += [random321mod4() for i in range(p-2*t)]
  L.sort()
  L = [(L[i]%4)-1 for i in range(p)]
  return Zx(L)

def keygen():
  while True:
    g = Zx([randomrange3()-1 for i in range(p)])
    if R3(g).is_unit(): break
  grecip = [nicemod3(lift(gri)) for gri in list(1/R3(g))]
  f = randomR()
  h = Rq(g)/(3*Rq(f))
  pk = encodeRq(h)
  return pk,encodeZx(f) + encodeZx(grecip) + pk

def encapsulate(pk):
  h = decodeRq(pk)
  r = randomR()
  hr = h * Rq(r)
  m = Zx([-nicemod3(nicelift(hr[i])) for i in range(p)])
  c = Rq(m) + hr
  fullkey = hash(encodeZx(r))
  return fullkey[:32] + encoderoundedRq(c),fullkey[32:]

def decapsulate(cstr,sk):
  f,ginv,h = decodeZx(sk[:191]),decodeZx(sk[191:382]),decodeRq(sk[382:])
  confirm,c = cstr[:32],decoderoundedRq(cstr[32:])
  f3mgr = Rq(3*f) * c
  f3mgr = [nicelift(f3mgr[i]) for i in range(p)]
  r = R3(ginv) * R3(f3mgr)
  r = Zx([nicemod3(lift(r[i])) for i in range(p)])
  hr = h * Rq(r)
  m = Zx([-nicemod3(nicelift(hr[i])) for i in range(p)])
  checkc = Rq(m) + hr
  fullkey = hash(encodeZx(r))
  if sum(r[i]==0 for i in range(p)) != p-2*t: return False
  if checkc != c: return False
  if fullkey[:32] != confirm: return False
  return fullkey[32:]

### Additional code ###
### Write To File  ###
import json

def byte2hex(bytestr):
    return ''.join( ["%02X" % ord(x) for x in bytestr]).strip()

kat_list = []
for i in range(100):
  pk,sk = keygen()
  c,k = encapsulate(pk)
  assert decapsulate(c,sk) == k
  kat = {"c": byte2hex(c), "k": byte2hex(k), "pk": byte2hex(pk), "sk": byte2hex(sk)}
  kat_list.append(kat)

output = json.dumps(kat_list, indent=4)
f = open("kat.json", "w")
f.write(output)
