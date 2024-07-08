# FASTA Duplicate Sequence Finder

This tool processes FASTA files to identify and report duplicate sequences. Created for users in the Proteomics STP at The Francis Crick Institute but suitable for general use.


## Features

- Reads FASTA format files
- Identifies sequences that appear multiple times with different IDs
- Generates a report of all duplicate sequences found
- Works with both nucleotide and protein sequences
- Cross-platform compatibility (Windows, macOS)

## Installation

### Pre-compiled Binaries

For ease of use, pre-compiled binaries are available for Windows and macOS:

- [Download for Windows](https://github.com/Jack-Coutts/FastaDuplicates/releases/download/v1.0.0/fasta_duplicate_finder_windows.exe)
- [Download for macOS](https://github.com/Jack-Coutts/FastaDuplicates/releases/download/v1.0.0/fasta_duplicate_finder_mac)

Some systems may not allow you to run executables downloaded from the web. In this case, you can download a zipped 
folder containing both executables [here](https://github.com/Jack-Coutts/FastaDuplicates/raw/main/binary%20executables.zip).

### Usage

1. Run the executable:
   * On Windows: Double-click the .exe file or run it from the command line.
   * On macOS/Linux: Double-click on the file or run it from the terminal.

2. When prompted, enter the full path to your FASTA file:
    ```plaintext
   Please enter the path to your FASTA file: /path/to/your/file.fasta
   ```
3. The program will process the file and create a new file in the same directory as your input file, with "_duplicates.txt" appended to the original filename.

4. The program will display a message indicating where the output file has been saved:
    ```plaintext
   Duplicate sequences written to /path/to/your/file_duplicates.txt
   ```
5. Open the output file to view the results. E.g.

    ```plaintext
    Found 2 duplicate sequences:
    
    Duplicate sequence found:
      ID: >seq1
      ID: >seq2
      Sequence: ATCGATCG
    
    Duplicate sequence found:
      ID: >seq3
      ID: >seq4
      ID: >seq5
      Sequence: GCTAGCTA
    ```