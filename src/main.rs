use mlua::prelude::*;

fn main() -> LuaResult<()>
{
    let lua = Lua::new();

    let map_table = lua.create_table()?;
    map_table.set(1, "one")?;
    map_table.set("two", 2)?;

    lua.globals().set("map_table", map_table)?;

    let f = lua.load(r#"
i = 0
yvel = 0.0
y = 0.0
delta = 0.0001
gravity = 9.8

for i = 0,50000000 do
    yvel = yvel + delta*gravity*0.5
    y = y + yvel*delta
    yvel = yvel + delta*gravity*0.5
    
    i = i + 1
end

io.write(y)
    "#).into_function()?;
    
    let start = std::time::Instant::now();
    println!("running function...");
    f.call(())?;
    let elapsed_time = start.elapsed();
    println!("time: {}", elapsed_time.as_secs_f64());

    Ok(())
}