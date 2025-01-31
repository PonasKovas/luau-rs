// This C++ shim provides wrappers around fallible luau functions
// catching the exceptions and instead returning something closer to a rust's result type

#include "../vendor/luau/VM/include/lua.h"

namespace Shim {

// Functions that returned void originally, just return a lua_Status now.
lua_Status lua_createtable(lua_State* L, int narr, int nrec) {
    
}

}
