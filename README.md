# PrefixSpan Algorithm with Rust Implementation (not optimized yet)

This is my rust implementation of the PrefixSpan algorithm to mine sequential pattern from clickstream database, as a project to get myself familiar with Rust.    
A clickstream database is a database of sequences consisting of ordered events or actions.  
The input clickstream database is in SPMF format:  
```
3 -1 5 -1 2 -1 3 -1 1 -1 -2
4 -1 1 -1 2 -1 6 -1 6 -1 -2
3 -1 2 -1 1 -1 -2
5 -1 5 -1 -2
```
  
Where:  
Each line is a sequence of the database.  
Each positive integer is corresponding to an event or action.  
`-1` is the separator between events (actions).  
`-2` is the indicator of the end of the sequence.
  
For more information on PrefixSpan Algorithm and the sequential pattern mining problem, please examine this link [SPMF](https://www.philippe-fournier-viger.com/spmf/index.php?link=algorithms.php).
