#pragma once

#include <stdint.h>

static const uint8_t tiles_index [] =
{
{# Write tile indexes #}
{% for tile in data %}
    {{ tile.index | hex }},
{% endfor %}
};

static const uint8_t tiles_attr [] =
{
{# Write tile attributes #}
{% for tile in data %}
    {{ tile.attr | hex }},
{% endfor %}
};
