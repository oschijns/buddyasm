# TileSet encoder

While using tiles and sprites, each retro console has its own encoding scheme 
for graphical assets. This tool is intended to take a modern image format 
(bitmap, PNG, Aseprite, Tiled). And convert it into a tileset and source code 
so that it can be directly used in a retro console game project without further 
manual work. 

## Supported hardwares
- *NES / Famicom*
- *SNES / Super Famicom*
- *GameBoy & GameBoy Color*
- *PC-Engine*
- *WonderSwan*
- *MasterSystem*
- *Genesis / MegaDrive*
- *NeoGeo Pocket*
- *NeoGeo*

## Usage
This tool uses a manifest file to collect all the graphical assets of your 
project and combine them into a unified tileset. To reconstruct the initial 
artwork on the target console, you should define additional *Tera* (*Jinja*) 
templates to properly layout the tile indexes and attributes.
