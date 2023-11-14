#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>

struct Record {
    std::string field1;
    std::string field2;
    // Add more fields as per your CSV structure
};

std::vector<Record> parseCSV(const std::string& filepath) {
    std::vector<Record> records;
    std::ifstream file(filepath);
    
    if (file.is_open()) {
        std::string line;
        while (std::getline(file, line)) {
            std::stringstream ss(line);
            std::string field1, field2;
            std::getline(ss, field1, ',');
            std::getline(ss, field2, ',');
            records.push_back({field1, field2});
        }
        file.close();
    }
    
    return records;
}

int main() {
    std::vector<Record> records = parseCSV("path_to_your_file.csv");
    for (const auto& record : records) {
        std::cout << "Field1: " << record.field1 << ", Field2: " << record.field2 << std::endl;
    }
    return 0;
}