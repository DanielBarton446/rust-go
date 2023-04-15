# rust-go
This is a go application that has integration with OGS. 


## Installing GH Hooks

In the folder called `precommit_installable_hooks`, there are 
hooks that you can 'opt-in' to for QOL. 

To do this:
`cd .git/hooks && ln -s <name_of_hook> <path to hook>`

E.g.
`cd .git/hooks && ln -s ../../precommit_installable_hooks/pre-commit ./pre-commit`
