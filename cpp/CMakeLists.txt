cmake_minimum_required(VERSION 3.22)
# Global Variables
set(TARGET aoc2022)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Set the etape value
set(DAY "1" CACHE STRING "Enter the day to compile.")

# Get the sources and headers
file(GLOB SOURCES "${PROJECT_SOURCE_DIR}/day${DAY}/src/*.cpp")
add_executable(${TARGET} ${SOURCES})
target_include_directories(${TARGET} PUBLIC "${PROJECT_SOURCE_DIR}/day${DAY}/include/")

# Adding compilation option depending on the OS 
if(MSVC)
  target_compile_options(${TARGET} PUBLIC /W3 /WX /DEBUG )
else()
  target_compile_options(${TARGET} PUBLIC -Wall -Wextra -Wpedantic -Werror -lstdc++)
endif()

