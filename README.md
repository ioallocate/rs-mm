# rs-mm

a memory manager which binds to nt-read/write, to easily r/w of the desired process, which also offers a standalone addition of mouse movement.

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

          foo3.write::<u8>(u_address, &u_value)?;

mouse instance creation ->

            let mouse = Mouse::new();
            
fn move_to usage (instant) ->

          mouse.move_to(500, 300, None, None)?;
          
fn move_to usage (smooth with 30 steps) ->

          mouse.move_to(500, 300, Some(30), None)?;
          
fn move_to usage (with sensitivity 1.5x) ->

          mouse.move_to(500, 300, None, Some(1.5))?;
          
fn move_to usage (smooth + sensitivity) ->

          mouse.move_to(500, 300, Some(25), Some(0.8))?;
          
fn left_click usage ->

          mouse.left_click()?;
          
fn right_click usage ->

          mouse.right_click()?;
          

## Include 

include in cargo.toml -> 

           [dependencies] 
           mm = { path = "path_to_crate" }
           
in-file include ->

           use mm::{p_handle, mmg}; //example


## Details

 - Process Elevation is not required
 - Binds the functions

          read            -> NtReadVirtualMemory
          write           -> NtWriteVirtualMemory
          mouse control   -> mouse_event (Windows API)

                  
