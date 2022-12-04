fn main() {
       // the mut keyword allows the variables to be modified later on.
       let mut x = 4;
       println!("x is: {}", x);
   
       // Name shadowing
       {
           // in the interior scope we can use variables from outside the scope, but not the other way round.
           println!("interior scope start");
           let x = 9;
           println!("x is: {}", x);
   
           // for example
           let x = x - 2;
           println!("x is: {}", x);
           println!("interior scope end");
   
           // but the value of x will only be changed inside and not outside the scope.
       }
       // a way to change the variable is to use the variable and add another value to it.
       // the reason that this line below doesn't print 12 is because the x = 9 line is in a different scope and the compiler just adds 3 to 4 instead of 3 to 9.
       x = x + 3;
       println!("x is: {}", x);
       // another way to change the variable is to define the variable again.
       let x = 6;
       println!("x is: {}", x);
}