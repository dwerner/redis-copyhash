#[macro_use]
extern crate redis_command_gen;

#[macro_use]
extern crate redis_module_sys;
extern crate libc;

use redis_module_sys::redis::{
    Redis,
    RedisCommandAttrs,
    RedisCommand,
    Reply,
};

use redis_module_sys::error::CellError;

#[derive(RedisCommandAttrs)]
#[command(name="ch.copyhash", flags="write", static_handle="CH_COPY_HASH_CMD")]
struct CopyHashCommand;
impl RedisCommand for CopyHashCommand {
    fn run(&self, r: Redis, args: &[&str]) -> Result<(), CellError> {
        let usage_str = "Usage: ch.copyhash <src_key> <target_key>";

        if args.len() != 3 {
            return Err(redis_error!("Wrong number of arguments to command.\n{}", usage_str));
        }

        let src = args[1];
        let dest = args[2];

        let maybe_fields = get_all_fields(&r, src);
        let mut ctr = 0;
        match maybe_fields {
            Ok(entries) => {
                ctr = entries.len();
                for (field, val) in entries {
                   try!(set_field(&r, dest, &field, &val));
                }
            },
            Err(e) => return Err(redis_error!("no entries found under key: {}", src))
        };

        self.reply(
            &r,
            &Reply::String(
                format!("OK - {} fields copied from {} to {}", ctr, src, dest)
            )
        )
    }
}

fn get_all_fields(r: &Redis, hash: &str) -> Result<Vec<(String, String)>, CellError>{
    match r.call("HGETALL", &[hash]) {
        Ok(Reply::Array(contents)) => {
            println!("read len: {} contents: {:?}", contents.len(), contents);
            let mut entries = Vec::with_capacity(contents.len() / 2 as usize);
            for chunk in contents.chunks(2) {
                if let Reply::String(ref field) = chunk[0] {
                    if let Reply::String(ref val) = chunk[1] {
                        entries.push((field.to_string(), val.to_string()));
                    }
                }
            }
            Ok(entries)
        },
        Ok(Reply::Nil) => Ok(Vec::new()),
        Ok(r) => return Err(redis_error!("unexpected Reply reading hash {} :  {:?}", hash, r)),
        Err(e) => return Err(redis_error!("unable to read hash {}", hash))
    }
}

fn set_field(r: &Redis, key: &str, field: &str, val: &str) -> Result<(), CellError> {
    match r.call("HSET", &[key, field, val]) {
        Ok(Reply::Integer(i)) => {
            println!("copied hash key {} -> {}", key, val);
            Ok(())
        },
        Ok(r) => Err(redis_error!("unexpected response: {:?}", r)),
        Err(e) => Err(
            redis_error!("error while setting {} {} {}", key, field, val)
        )
    }
}

redis_module!(copy_hash, 1, CH_COPY_HASH_CMD);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

