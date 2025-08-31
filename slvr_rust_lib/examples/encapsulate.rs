use slvr_rust_lib::encapsulation::Encapsulates;
use slvr_rust_lib::encapsulation::encapsulate;
use std::collections::HashMap;

// cargo run --package slvr_rust_lib --example encapsulate --features encapsulation

#[cfg(not(feature = "encapsulation"))]
compile_error!("This example requires the encapsulation feature.");

fn main() {



    let mut user_cache: Cache<User> = Cache::new();
    let mut song_cache: Cache<Song> = Cache::new();

    user_cache.set(User {
        name: "bob".to_string(),
        encapsulated_cache_id: CacheId::new(0),
    });

    user_cache.set(User {
        name: "alice".to_string(),
        encapsulated_cache_id: CacheId::new(1),
    });

    song_cache.set(Song {
        name: "give you up".to_string(),
        author: "rick".to_string(),
        encapsulated_cache_id: CacheId::new(0),
    });

    fn print_user(cache: &mut Cache<User>, id: u64) {
        println!("got user: {} with id {}", cache.get(id).unwrap().name, id);
    }
    fn print_song(cache: &mut Cache<Song>, id: u64) {
        let song = cache.get(id).unwrap();
        println!("got song: {} by {} with id {}", song.name, song.author, id);
    }

    print_user(&mut user_cache, 0);
    print_user(&mut user_cache, 1);
    print_song(&mut song_cache, 0);
}

#[encapsulate(CacheId)]
struct User {
    name: String,
}

#[encapsulate(CacheId)]
struct Song {
    name: String,
    author: String,
}

#[repr(transparent)]
struct CacheId {
    id: u64,
}
impl CacheId {
    fn new(id: u64) -> Self {
        CacheId { id }
    }
}

struct Cache<T: Encapsulates<CacheId>> {
    pub(self) backing: HashMap<u64, T>,
}

impl<T: Encapsulates<CacheId>> Cache<T> {
    pub fn set(&mut self, item: T) {
        let id: &CacheId = item.encapsulated_get();
        self.backing.insert(id.id, item);
    }

    fn get(&mut self, id: u64) -> Option<&T> {
        self.backing.get(&id)
    }

    fn new() -> Cache<T> {
        Cache {
            backing: HashMap::new(),
        }
    }
}
