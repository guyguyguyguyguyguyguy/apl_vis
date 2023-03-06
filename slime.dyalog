conga←'/opt/mdyalog/18.2/64/unicode/lib/conga34_64.so'
⎕RL ← 1
⎕NA 'I4 ',conga,'|getpid'
)COPY conga 
r ← Conga.Init ''
r.Clt 'C1' 'localhost' 7342 'Raw' 
sender ← {r.Send 'C1' ⍵}

x y z ← 60 60 3
shape ← x y z
size ← x × y × z

sender shape

RR ← ¯1+?x y⍴ 2 

slime ← {}

∇ f ← OneStep arr
  rgb ← x y z ⍴ 3/arr
  te  ← ⎕JSON (size ⍴ rgb)
  sender ⊢te
  _←⎕dl÷5
  f ← slime arr  
∇

OneStep ⍣≡ RR
