defmodule PGP do
  use Rustler,
    otp_app: :ex_pgp,
    crate: :ex_pgp

  def generate(_key_size), do: :erlang.nif_error(:nif_not_loaded)
  def created_at(_pem), do: :erlang.nif_error(:nif_not_loaded)
  def export_public_key(_pem), do: :erlang.nif_error(:nif_not_loaded)
  def export_rsa_pubkey(_pem), do: :erlang.nif_error(:nif_not_loaded)
  def encrypt(_pem, _data), do: :erlang.nif_error(:nif_not_loaded)
  def decrypt(_pem, _data), do: :erlang.nif_error(:nif_not_loaded)
  def fingerprint(_pem), do: :erlang.nif_error(:nif_not_loaded)
end
