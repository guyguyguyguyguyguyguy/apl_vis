conga←'/opt/mdyalog/18.2/64/unicode/lib/conga34_64.so'
⎕RL ← 72
⎕NA 'I4 ',conga,'|getpid'
)COPY conga 
r ← Conga.Init ''
r.Clt 'C1' 'localhost' 7342 'Raw' 
sender ← {r.Send 'C1' ⍵}

x y z ← 120 100 3
shape ← x y z
size ← x × y × z

sender ⍕ shape

life ← {⊃1 ⍵ ∨.∧ 3 4 = +/ +⌿ 1 0 ¯1 ∘.⊖ 1 0 ¯1 ⌽¨ ⊂⍵}
RR ← ¯1+?x y⍴ 2 

gen ← {(life⍣⍵)⍺}
⍝ Take R to gen 100
⍝ R∘gen 100

⍝ Store R over 100 generations
⍝ R∘gen¨ ⍳100

⍝ createlife ⍣≡ RR ⍝ this prints the gol progression endlessly

∇ f ← OneStep gol
  rgb ← x y z ⍴ 3/gol ⍝ Need to make sure this works as intended
  te  ← ⎕JSON (size ⍴ rgb)
  sender ⊢ te  ⍝ send each row at a time due to max packet size
  _←⎕dl÷10
  f ← life gol
∇

OneStep ⍣≡ RR
⍝ OneStep RR


