v x = 0
w x < 101 {
    p(x)
    i x % 3 == 0 {
        p("Fizz")
    }
    i x % 5 == 0 {
        p("Buzz")
    }
    x = x + 1
}