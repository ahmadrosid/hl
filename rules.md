# Rules
The development of this software is based on the rules that we have on folder `rules` from the root of this project. The lexers generated based on the rules in yaml format. Most of the hacky thing for generating the lexers is inside `condition` rules. 

Here's the documentation of the rules available for our lexers.
- constant
- single_constant
- keyword
- single_keyword
- skip
- entity
- entity_tag
- prefix
- slash_comment
- condition
- slash_star_comment
- var
- entity_prefix
- entity_suffix
