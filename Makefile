
rust:
	cargo run

ruby:
	ruby main.rb

CXX = g++-9
CXXFLAGS = -Wall -std=c++17
TARGET = a.out
SRC = main.cpp
cpp: $(SRC)
	@$(CXX) $(SRC) -o $(TARGET) $(CXXFLAGS)
	@./$(TARGET)
	@make clean

.PHONY: clean
clean:
	@$(RM) -f $(TARGET) *.o
