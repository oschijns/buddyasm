#pragma once

static const uint8_t tiles_index [] =
{
{% for tile in data %}
    {{ tile.tile | hex }},
{% endfor %}
};

static const uint8_t tiles_attr [] =
{
{% for tile in data %}
    {{ tile | attr_nes }},
{% endfor %}
};
