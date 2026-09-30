#once

{# Generate bytes to reconstruct the artwork #}

#bank consts
artwork:
    .WIDTH  = {{ dim[0] }}
    .HEIGHT = {{ dim[1] }}

    .indexes:
#d {% for tile in data -%}{{ tile.index | hex }}, {% endfor %}
        ..len = $ - .indexes

    .attributes:
#d {% for tile in data -%}
    {%- if (loop.index0 % 2 == 0) and ((loop.index0 // 10) % 2 == 0) -%}
        {{ tile.attr | hex }},
    {%- endif -%}
{%- endfor %}
        ..len = $ - .attributes
