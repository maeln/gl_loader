#include <stdio.h>
#include <windows.h>
#include <assert.h>

void *gl_get_proc_addr(const char *name)
{
  static HMODULE opengl32module = NULL;
  static PROC(WINAPI * wgl_get_proc_address)(LPCSTR name) = NULL;
  if (!wgl_get_proc_address)
  {
    if (!opengl32module)
    {
      opengl32module = LoadLibraryA("opengl32.dll");
    }
    wgl_get_proc_address = (PROC(WINAPI *)(LPCSTR))GetProcAddress(
        opengl32module, "wglGetProcAddress");
    assert(wgl_get_proc_address);
  }
  void *ptr = (void *)wgl_get_proc_address(name);
  if (ptr == 0 || (ptr == (void *)1) || (ptr == (void *)2) ||
      (ptr == (void *)3) || (ptr == (void *)-1))
  {
    if (opengl32module == NULL)
    {
      opengl32module = LoadLibraryA("opengl32.dll");
      assert(opengl32module);
    }
    ptr = (void *)GetProcAddress(opengl32module, name);
  }
  return ptr;
}