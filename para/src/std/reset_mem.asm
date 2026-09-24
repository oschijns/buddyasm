; Can be included more than once

; Reset memory
#bank zptmp
#addr START_ZP
#bank memtmp
#addr START_MEM

; Default to bank header afterward
#bank header
#addr 0
