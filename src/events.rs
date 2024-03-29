use input::event::pointer::PointerButtonEvent;
use input::event::PointerEvent;
use input::{Libinput, LibinputInterface};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

extern crate libc;
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        unsafe {
            let _ = File::from(fd);
        }
    }
}

pub fn listen_for_mouse_button_events(handle_event: fn(PointerButtonEvent) -> ()) {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            match event {
                input::Event::Pointer(pointer_event) => match pointer_event {
                    PointerEvent::Button(x) => handle_event(x),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
