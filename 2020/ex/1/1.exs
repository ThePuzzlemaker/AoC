input = File.read!("1.input")
  |> String.split("\n", trim: true)
  |> Enum.map(&String.to_integer/1)
  |> Enum.dedup

defmodule Permutations do
  def of(stream, i), do: of(stream, [], i)
  def of(_, cons, 0), do: [cons]
  def of(stream, cons, dec) do
    Stream.map(stream, &(cons ++ [&1]))
    |> Stream.flat_map(&(of(stream, &1, dec - 1)))
  end
end

IO.write("2 elements: ")
input
|> Permutations.of(2)
|> Enum.find(&(Enum.sum(&1) == 2020))
|> Enum.reduce(1, &*/2)
|> Integer.to_string
|> IO.puts

IO.write("3 elements: ")
input
|> Permutations.of(3)
|> Enum.find(&(Enum.sum(&1) == 2020))
|> Enum.reduce(1, &*/2)
|> Integer.to_string
|> IO.puts

