extern crate rand;

#[derive(Debug)]
struct Monster {
  name: String,
  health: i8
}

#[derive(Debug)]
struct Player {
  health: i8
}

struct Status {
  player_turn: bool,
  player_alive: bool,
  monster_alive: bool
}

impl Monster {
  fn attack(&self, player: &mut Player){
    let attack = (rand::random::<u8>() % 100) + 1;

    println!("{} hits you for {}!", self.name, attack);
    player.health -= attack as i8;
  }
}

impl Player {
  fn attack(&self, monster: &mut Monster){
    println!("You hit {}!", monster.name);
    monster.health -= 10;
  }
}

fn main(){
  let mut monster = Monster {
    name: "Evil Goo Cube".to_string(),
    health: 50
  };

  let mut player_1 = Player {
    health: 100
  };

  let mut status = Status {
    player_turn: true,
    player_alive: true,
    monster_alive: true
  };

  fn begin_turn(mut status: &mut Status, mut monster: &mut Monster, mut player: &mut Player){
    if status.player_turn == true {
      player.attack(&mut monster);

      if monster.health <= 0 {
        status.monster_alive = false;
      }

      status.player_turn = false;
      check_status(status, monster, player);
    } else {
      monster.attack(&mut player);

      if player.health <= 0 {
        status.player_alive = false;
      }

      status.player_turn = true;
      check_status(status, monster, player);
    }
  }

  fn check_status(mut status: &mut Status, monster: &mut Monster, player: &mut Player){
    if status.player_alive == false {
      println!("===============================");
      println!("You have been defeated by {}", monster.name);
    } else if status.monster_alive == false {
      println!("===============================");
      println!("You have defeated {}", monster.name);
    } else {
      println!("Player health: {}", player.health);
      println!("Monster health: {}", monster.health);
      println!("===============================");
      begin_turn(status, monster, player);
    }
  }

  begin_turn(&mut status, &mut monster, &mut player_1);
}
