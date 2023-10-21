local DNA = {}

function DNA:new(chain)
   DNA.nucleotideCounts = { A = 0, T = 0, C = 0, G = 0 }
   for i = 1, #chain, 1 do
       local key = chain:sub(i, i)
       assert(DNA.nucleotideCounts[key], "Invalid Sequence")
       DNA.nucleotideCounts[key] = DNA.nucleotideCounts[key] + 1
   end
   return DNA
end

function DNA:count(nucleotide)
    local result = DNA.nucleotideCounts[nucleotide]
    assert(result, "Invalid Nucleotide")
    return result
end

return DNA
