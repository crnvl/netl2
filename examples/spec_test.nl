f test_fn {
    v test = 20

    w test > 0 {
        p(test)
        test = test - 1

        i test == 10 {
            p("test is 10")
        }
    }
}

f nested_fn {
    v test = 3
    p(test)
    test_fn!
}
nested_fn!
