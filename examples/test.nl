v var_name = 2

w var_name == 2 {
    p("Hello World!")
    var_name = 3
}

i var_name != 2 {
    main(var_name)
}

f main(param) {
    p("Main function!")
    r(param)
}

main(var_name)