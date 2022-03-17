# pw-py service
Pathway API service

*API endpoint is hosted at https://pw-rs.gton.capital/rpc*

Method ```base-price``` (https://pw-rs.gton.capital/rpc/base-price) 

Method ```owned/base-pool-lps``` (https://pw-rs.gton.capital/rpc/owned/base-pool-lps) 

Method ```owned/usd-pool-lps``` (https://pw-rs.gton.capital/rpc/owned/usd-pool-lps) 

Method ```base-liquidity``` (https://pw-rs.gton.capital/rpc/base-liquidity)

Method ```usd-liquidity``` (https://pw-rs.gton.capital/rpc/usd-liquidity)

Method ```base-pool-lps``` (https://pw-rs.gton.capital/rpc/base-pool-lps)

Method ```usd-pool-lps``` (https://pw-rs.gton.capital/rpc/usd-pool-lps)

Method ```gc-pol``` (https://pw-rs.gton.capital/rpc/gc-pol)

Method ```pw-model-peg-with-pol-mln``` (https://pw-rs.gton.capital/rpc/pw-model-peg-with-pol-mln?pol=0&gc_floor=0&gc_bias=0&gc_max_p=0&gc_max_l=1)
This method supports multiple params.
Example with default param values:
```
pol 
gc_floor  =  2.05, # floor price
gc_bias  =  1.7,  # noise parameter
gc_max_p  =  600.0 # gc max P parameter
gc_max_l  =  550.0 # gc max L parameter
```

Basically, server calculates this:
```python
return max(gcFloor, gcBias + (gcMaxP * _pol / gcMaxL))
```

Check the response:
https://pw-rs.gton.capital/rpc/pw-model-peg-with-pol-mln?pol=1.1&gc_floor=2.05&gc_bias=1.7&gc_max_p=6040.0&gc_max_l=550


![Pathway PWPeg(t) function](https://i.imgur.com/oajBYQV.png)

Method ```gc-current-peg-usd``` (https://pw-rs.gton.capital/rpc/gc-current-peg-usd)
Method ```gc-current-peg-base``` (https://pw-rs.gton.capital/rpc/gc-current-peg-base)

Method ```base-to-usdc-price``` (https://pw-rs.gton.capital/rpc/base-to-usdc-price)
Method ```base-to-quote-price``` (https://pw-rs.gton.capital/rpc/base-to-quote-price)
