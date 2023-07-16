v comp_one = 1
v comp_two = 2
v comp_three = 3
v comp_four = 3

i comp_one == comp_two | comp_three != comp_four {
    p("worked!")
}
p("done")