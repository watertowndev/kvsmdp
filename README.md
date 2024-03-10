# KVSMDP
Meter Data Processing Utility for converting KVS/CyberQuery-generated fixed-width output files into CSV and JSON files.

## Purpose
This utility converts CyberQuery-generated output files into CSV and JSON. 

The CSV and JSON output files are intended to be imported into other applications or databases. 

The data library components are intended to be sufficiently generic that they may be useful on their own; however, no testing has be done on this front.

## Usage
    kvsmdp.exe <inputfile> <csvfile> <jsonfile>
    <inputfile>  Input file to process
    <csvfile>    Output CSV file to create
    <jsonfile>   Output JSON file to create
    <logfile>    Log file to write to
Output files are overwritten if they exist.

