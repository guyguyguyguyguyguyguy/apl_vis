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

life ← {⊃1 ⍵ ∨.∧ 3 4 = +/ +⌿ 1 0 ¯1 ∘.⊖ 1 0 ¯1 ⌽¨ ⊂⍵}
RR ← ¯1+?30 30⍴ 2 ⍝ Not so random...

gen ← {(life⍣⍵)⍺}
⍝ Take R to gen 100
⍝ R∘gen 100

⍝ Store R over 100 generations
⍝ R∘gen¨ ⍳100

⍝ createlife ⍣≡ RR ⍝ this prints the gol progression endlessly


⍝ This works!
∇ f ← OneStep gol
  rgb ← x y z ⍴ 3/gol
  te  ← ⎕JSON (size ⍴ rgb)
  sender te
  _←⎕dl÷5
  f ← life gol
∇

OneStep ⍣≡ RR


⍝ :While RR ≢ end
⍝   sender (⎕JSON (900⍴RR))
⍝   _←⎕dl÷8
⍝   RR ← life RR
⍝ :EndWhile


