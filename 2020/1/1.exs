body = File.read!("1.input")
elems = String.splitter(body, "\n", trim: true)
        |> Stream.map(&elem(Integer.parse(&1), 0))
        |> Stream.dedup

defmodule Permutations do
  def of(stream, i), do: of(stream, [], i)
  def of(_, cons, 0), do: [cons]
  def of(stream, cons, dec) do
    Stream.map(stream, &(cons ++ [&1]))
    |> Stream.flat_map(&(of(stream, &1, dec - 1)))
  end
end

IO.write("2 elements: ")
elems
|> Permutations.of(2)
|> Enum.find(&(Enum.sum(&1) == 2020))
|> Enum.reduce(1, &*/2)
|> Integer.to_string
|> IO.puts

IO.write("3 elements: ")
elems
|> Permutations.of(3)
|> Enum.find(&(Enum.sum(&1) == 2020))
|> Enum.reduce(1, &*/2)
|> Integer.to_string
|> IO.puts
