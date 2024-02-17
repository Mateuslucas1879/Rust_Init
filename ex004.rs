fn main (){
    let x = 5; 
    let x =x+1;
    {
        let x =x*2;
        println!("O valor e x no escopo:{x}");
    }

    println!("O valor e x e:{x}");
}