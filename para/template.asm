#once

{# Generate bytes to reconstruct the artwork #}

#bank consts
artwork:

    .indexes:
#d {% for tile in data -%}{{ tile.index | hex }}, {% endfor %}
        ..len = $ - .indexes

    .attributes:
#d {% for tile in data -%}{{ tile.attr | hex }}, {% endfor %}
        ..len = $ - .attributes

