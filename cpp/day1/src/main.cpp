#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <stdexcept>
#include <algorithm>

struct Elf {
public:
    Elf()
        : pocket({})
    {
    }

    void add(uint64_t item)
    {
        pocket.push_back(item);
    }

    uint64_t getTotal() const
    {
        uint64_t total(0);
        for (const auto& item : pocket)
            total += item;
        return total;
    }

    void clear()
    {
        pocket.clear();
    }

private:
    std::vector<uint64_t> pocket;
};

std::ifstream open_file();

int main()
{
    std::ifstream file;
    std::string file_content("");
    std::vector<uint64_t> elvesTotal;

    try
    {
        file = open_file();
    } 
    catch (...) 
    {
        std::cerr << "Could not open file." << std::endl;
        return -1;
    }

    // Gathering each elf into the elves vector
    Elf tempelf;
    while (std::getline(file, file_content))
    {
        try {
            tempelf.add(std::stoull(file_content));
        }
        catch(std::exception ex)
        {
            elvesTotal.push_back(tempelf.getTotal());
            tempelf.clear();
        }
    }

    std::sort(elvesTotal.begin(), elvesTotal.end() , [](uint64_t a, uint64_t b) { return a>b; });
    std::cout << "Elf Max: " << *elvesTotal.begin() << std::endl;
    uint64_t maxThree(0);
    for(auto it = elvesTotal.begin() ; it != elvesTotal.end() ; it++)
    {
        maxThree += *it;
        if(it == elvesTotal.begin()+2)
            break;
    }
    std::cout << "Three elves total: " << maxThree << std::endl;
    return 0;
}

std::ifstream open_file()
{
    std::ifstream file;
    file.open("elves");
    if (!file.is_open())
        throw -1;
    return file;
}