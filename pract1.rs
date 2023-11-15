fn main_first(){
    //c ! это макрос. делает то же что функция,
    // но в compile Time
    println!("{} {}", 42, 43);
    //rust дружит с unicode
    println!("привет, Rust");
}

/*
КОМПИЛЯЦИЯ
----------
rustc hello.rs -o hello
./hello

СБОРКА
------
Руками компилятор лучше не писать. Можно использовать систему сборки cargo.

Создание проекта:
cargo init project_name

Компиляция:
cargo build

Сборка и запуск:
cargo run

ФОРМАТИРОВАНИЕ:
cargo fmt

ПРОВЕРКА БЕЗ СОЗДАНИЯ БИНАРНИКА: 
cargo check

ЛИНТЕР (предлагает упрощать код и всякое такое):
cargo clippy
*/

/*
  usize -- беззнаковый тип размера указателя

  struct -- как классы в с++, держат данные

  конструкторов нет, для создания делаем статическая функцию, 
  возвращающую структуру
 */

 struct GameOfLife{
    //компилятор может класть данные как хочет
    //если его не попросить класть как в С
    n: usize,
    m: usize,
    data: Vec<Vec<bool>>,
 }

 /*
  impl -- описание методов над структурой
  */
/**
 * ПАКЕТЫ
 * ------
 * Случайной генерации в Rust нет. Можно скачать пакет с сайта: 
 * crates.io 
 * 
 * Для случайных чисел rand (cargo add rand или добавить в cargo.toml)
 * ...
 * [dependencies]
 * rand = "0.2.5"
 * ...
 */

use rand::random
impl GameOfLife{
    //этот метод статический
    fn new_random(n: usize, m: usize) -> Self{
        //тут макрос просто для удобства
        //тут можно и круглые скобки, и фигурные
        //а можно и точки с запятой убрать
        //mut -- изменяемые данные
        let mut data: = vec![vec![false; m]; n];
        for x in 0 .. n{
            for y in 0 .. m{
                //random сам вывел тип, который ему возвращать
                //по типу data
                data[x][y] = random()
                //rand::random();
            }
        }
        /**
         * если имя совпадает с именем поля, то можно 
         * без двоеточий. Если нет -- то нет
         * return GameOfLife {n: n, m: m, data: data}
         */

        //return можно не писать
        //вернется последнее выражение без ;

        //GameOfLife {n: n, m: m, data: data}
        Self{n, m, data}
    }

    //тут нет mut -- ссылка константана!
    //принимает self -- он не статичный!
    fn print(&self){
        for x in self.n{
            for y in self.m{
                //после if нет точки с запятой -- мы думаем, что 
                //if -- expression и он возвращает то, что вышло
                print!("{}", if self.data[x][y] {"."} else {"#"})
            }
            println!();
        }
    }

    fn alive_neighbors(&self, x: usize, y: usize){
        for dx in -1 .. = 1{
            for dy in -1..=1{
                if(dx == 0 && dy == 0){
                    continue;
                }
                //нет неявного приведения типов!
                let nx = (x as isize + dx + self.n as isize) % (self.n as isize);
                //shadowing, такое принято
                let nx = nx as usize; 
                //...same for y
                if self.data[nx][ny]{
                    //ind и dec отсутствуют
                    neighbors += 1;
                }
            }
        }

    }
    
    fn next(&mut self){
        //так нельзя, это мув. это сделает в self пустоту
        //a self unmut
        //let mut new_data = self.data
        let mut new_data = self.data.clone();
        for x in 0.. self.n{
            for y in 0 ..self.m{
                //...
            }
        }
        //так нельзя если self не mut
        self.data = new_data;

    }
}
fn main(){
    let life = GameOfLife::new_random(30, 40);
    //спец конструкция для бесконечного цикла
    loop{
        life.print()
        life.next()
        //read_line - возвращает либо Т резульат, либо ошибку
        //except -- при ошибке завершит с сообщением 
        //unwrap -- делает с каким-то деофолтным сообщением
        stdin().read_line(&mut String::new()).expect("cannot")
    }
    life.print();
}

/**
 * pub -- показатель публичности. По дефолту приватное, но 
 * видимо в рамках модуля
 */