rules
⍝ Particle has 3 sensors

⍝     1   2   3
⍝      \  |  /
⍝       \ | /
  ⍝ 1) seperated bysensor angle
  ⍝ 2) sensor distance
  ⍝ 3) sensor size
  ⍝ 3) heading
  ⍝ 4) location

⍝ Particle leaves a trail that diffuses

⍝ Rules
  ⍝ Particle rotates based on the conc. of particles in each sensor:
    ⍝ highest in sensor 2, dont change heading
    ⍝ highest in left and right, turn randomly
    ⍝ highest in right and lowest in left, turn right
    ⍝ highest in left and lowest in right, turn left

⍝ Process:
  ⍝ 1) Sense each particles sensors
  ⍝ 2) rotate
  ⍝ 3) move
  ⍝ 4) deposite trail
  ⍝ 5) diffuse trail
  ⍝ 6) trail decay