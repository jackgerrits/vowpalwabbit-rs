#pragma once

#if defined _WIN32 || defined __CYGWIN__
#  ifdef VW_RS_BUILDING_DLL
#    ifdef __GNUC__
#      define DLL_PUBLIC __attribute__((dllexport))
#    else
#      define DLL_PUBLIC __declspec(dllexport)  // Note: actually gcc seems to also supports this syntax.
#    endif
#  else
#    ifdef __GNUC__
#      define DLL_PUBLIC __attribute__((dllimport))
#    else
#      define DLL_PUBLIC __declspec(dllimport)  // Note: actually gcc seems to also supports this syntax.
#    endif
#  endif
#  define DLL_LOCAL
#else
#  if __GNUC__ >= 4
#    define DLL_PUBLIC __attribute__((visibility("default")))
#    define DLL_LOCAL __attribute__((visibility("hidden")))
#  else
#    define DLL_PUBLIC
#    define DLL_LOCAL
#  endif
#endif

extern "C"
{
  static const int VW_STATUS_SUCCESS = 0;
  static const int VW_STATUS_FAIL = 1;

  struct VWWorkspace;

  DLL_PUBLIC int VWInitializeWorkspace(const char* const* symbols, int count, struct VWWorkspace** output_handle,
      const char** error_message) noexcept;
  DLL_PUBLIC int VWFreeWorkspace(struct VWWorkspace* workspace_handle, const char** error_message) noexcept;
  DLL_PUBLIC int VWFreeErrorMessage(const char* error_message) noexcept;

  DLL_PUBLIC int VWRunDriver(struct VWWorkspace* workspace_handle, const char** error_message) noexcept;
}

