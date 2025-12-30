# rs-mm

a memory manager which binds to nt-read/write, to easily r/w of the desired process

## Usage

pid retrieval (u32) ->

           let foo = p_handle::find_process_by_name("dprocess.exe")?;

chandle retrieval (p_handle) ->

           let foo1 = p_handle::open_by_pid(foo)?;

module base retrival (usize) ->

           let foo2 = foo1.get_module_base("dprocess.exe")?;

local instance creation (mmg) ->

          let foo3 = mmg::new(&foo);

fn read usage (usize) ->

          let foo4 = foo3.read::<usize>(u_address)?;

fn write usage (usize) ->

          foo3.write::<u8>(address, &value)?;
