require 'securerandom'

animals = %w[
  Aardvaark
  Buffalo
  Chinchilla
  Donkey
  Elephant
  Fox
  Gorilla
  Hamster
  Ibex
  Jackal
  Kangaroo
  Llama
  Marmoset
  Napu
  Opossum
  Panther
  Quokka
  Raccoon
  Sloth
  Tiger
  Uakari
  Vicuña
  Wombat
  Xerus
  Yak
  Zebra
]

contexts = %w[
  web
  ActionCable
  assets
  ActiveJob
]

requests = []

1000.times do
  if requests.none? || rand(requests.size * 3) == 0
    requests << [ contexts.sample, SecureRandom.uuid, animals.take(rand(animals.size) + 1).sort]
  end

  r = requests.sample
  animal = r.last.shift

  if r.first == "ActiveJob"
    puts "[#{r[0]}] [JobName] [#{r[1]}] #{animal}"
  else
    puts "[#{r[0]}] [#{r[1][0..4]}…] #{animal}"
  end

  requests = requests.select {|r| r.last.any? }
end
