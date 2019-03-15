# GL Loader

This (small) package aim to do *only one thing*: Provide a `get_proc_address` function to load OpenGL function pointer.

It will not provide any form of window or OpenGL context creation. You will have to handle them by yourself.

## Explanation

When using the OpenGL's headers, the function are just declaration, empty shell. 
Trying to call them will result in whatever happen when you try to use a null pointer as a function.

To really be able to use those functions, you need to load their pointer.
The OS will provide them for you by usually exposing a `getProcAddress` function (or equivalent) which take the OpenGL function name and output its pointer.

Therefor, this crate aim to provide a OS-independant way to load the OpenGL function pointer.

## Why 

The main idea is to be able to make a OpenGL-based engine that is complitely agnostic from the GUI/Windowing library used.
Let's say for exemple that you want to make an editor that use libGUI-X and package your final application with libWindow-Y. 
Both these system should handle the window and context creation but might have a way to load function pointer which is uncompatible or require you to do dirty hacks in your engine to handle both case.

By deffering the function pointer loading to this small library, you should be able to make your engine complitely agnostic as long as a proper OpenGL-context has been created and made current on the same thread.

## How to use

First you will need the following requirement:
1 - An OpenGL context has been created and made current
2 - You are calling the `get_proc_address` from *the same thread* has the one which owns the current context

Then, using the `gl` crate, you should be able to just do:
```
use gl_loader::loader::get_proc_address;

fn load_gl_symbol() {
    unsafe {
        gl::load_with(|symbol| get_proc_address(symbol) as *const _);
    }
}
```

Your function pointer should be loaded and you should be able to use OpenGL.

## Support

For now I only had the time to copy paste the code for Mac OS X and test it.
I will update the package to support at least windows and linux when I have the time.

## Goal

Right now, some of the dependencies are relying on the std.
The final goal is to make the package fully `#![no_std]` compatible.

## Credit

**glutin contributors**: Most of the code is straight out ripped from glutin.

**fizzer and ferris**: For giving me the idea and explaining some of the specific to me