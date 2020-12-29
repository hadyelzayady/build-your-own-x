#include <cstdlib>
#include "window_manager.hpp"

using ::std::unique_ptr;

int main(){
  unique_ptr<WindowManager> window_manager = WindowManager::Create();
  if(!window_manager){
    return EXIT_FAILURE;
  }

window_manager->Run();
return EXIT_SUCCESS;
}
