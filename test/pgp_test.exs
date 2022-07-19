defmodule PGPTest do
  use ExUnit.Case, async: true

  @private_key """
  -----BEGIN PGP PRIVATE KEY BLOCK-----

  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
  XegYj7aU/+xmOJSE7Yw=
  =RTPr
  -----END PGP PRIVATE KEY BLOCK-----
  """
  @public_key """
  -----BEGIN PGP PUBLIC KEY BLOCK-----

  mQENBGH9n0kBCAClgZJVbq8ji4z6uiufWnbfLh0Ur2QC3puNFGS2YKPBfXrTm2ky
  dE3U7yhrxbx12cyK8jv2ZBROHU21kscgWUUriq1y4iQ1XocWUqm4AYo8XBvbJC5W
  r/ZoEVuThJw/vjobgIYsPp4CTFeJdlihuxxtXqjwKBpcoj0m914lqj3g6l6w/TpE
  k8MPtihANhG6mUGaeGzvtiyfjhON6MhVY58vgqv2oP+B71X+kjTNbZeOALQdZfbR
  xXWD8+72e1PkhpU28EBW2TIp5Kh+VF48G71Om2uVQD4GBAWsa0QVitkGMl0YO5Qd
  NBy3JH5t6X4cprEQ9NR9XsX+S8nQ06RdVlQJABEBAAG0DEp1bXBXaXJlIERldokB
  TgQTAQoAOBYhBIH+yd1LMsO0xXcWKuLSRNDlJ7mvBQJh/Z9JAhsDBQsJCAcDBRUK
  CQgLBRYCAwEAAh4BAheAAAoJEOLSRNDlJ7mvfkIH/2cVjxPVt+DipkMowIWDE7mo
  ek07EhTHN7kKvNHxq9of1VvqUFaYG2seeLpsHB46cfR637r8Bse0SMtHfUbWnCXK
  jvDMHdZSPhY8oNrHpyY97e6zUvoHW6YcvEw67mm5qW+fHzVW0wmKLnm35HUw+wtj
  nIb9cxndCn2Jo0S2bRzT7aowPuHnLEQXejt6mCaKY571sOzYMFim4NbBuBiu9qVo
  JkwWo1BHyAujmRVPRA14CQqCldxmJsnlyPOv4ZlTe30b/jPJHKI+5HqHz0zDYO4W
  jrel7Ww5E/cBynJz2HqZUQGiCPwi5YNbIPWtGiC3rIl/qrl/3WWl8UtM77Xv76e5
  AQ0EYf2fSQEIAOvS+sH69d9A/6UHHMZWbX8YK8/RwzT4Q5+oZgT+h8QJ97bljrFu
  dcjDAglFJLvV8/bKx+NM2M+9yQ/B9SvudpSl3OzJim4+zZgZxq1eAu/blCewNVUW
  h63z/q9z4zTeRcmfKAoYPbMy0cNsP6KoOSnOTTAWj4N7uovWJFJhXR8UxP/mIiq1
  yoh9XyiA1EfRIVSKrY95mbdq5M8vFF6VAuOoj72uTyotXm+gJuirt6ACapL/wumW
  F5pj/8BFGan2vU/d1GZDYqUdUDmz/xH4wPqt2nMZ9LIO63ftvmj4QadtzsLLRO3G
  dfgfJ2RKOpBOEyy3hGEynjDwkg2pjCYDFtUAEQEAAYkBNgQYAQoAIBYhBIH+yd1L
  MsO0xXcWKuLSRNDlJ7mvBQJh/Z9JAhsMAAoJEOLSRNDlJ7mvDn4H/3s/e+Gz8qD+
  twI8fjYMeEsLFtQD/njL8ZRuyLvtytccUILPGuWM70NhjQk4C7gtq/FiLcOs4TVn
  wsf7Zl4d2kehA0AN04+GlgISkoP42Rzawi94rvvVQDylvLfbRHLCggy0c1gUCCRQ
  cuAYd4vHzJR/lG1c78UCHPNMaEwm8Z3tVbIJsZ8bIN9cysADuM5v3P5vc09vCjES
  MmpO6T05Ab9eIn3dP2zThNGrvHBfC/Ek5UgJQ/JMjvThBpQATG4rMb3JE4tY+/9r
  Od+EMRga3Yxptd3WQwNcMzIdyMR5BuNvELuQtH3mP5Tz9TGHONT3MFEaXegYj7aU
  /+xmOJSE7Yw=
  =i1R6
  -----END PGP PUBLIC KEY BLOCK-----
  """

  setup do
    %{public_key: @public_key, private_key: @private_key}
  end

  test "fingerprinting of key", %{private_key: key} do
    assert {:ok, fingerprint} = PGP.fingerprint(key)
    assert "81FEC9DD4B32C3B4C577162AE2D244D0E527B9AF" == Base.encode16(fingerprint)
  end

  test "generate public key from private pem", %{private_key: private_key} do
    # NB: metadata such as timestamps will change, so we can't directly compare the public keys
    plaintext = "science fiction/double feature"

    assert {:ok, public_key} = PGP.export_public_key(private_key)
    assert {:ok, encrypted} = PGP.encrypt(public_key, plaintext)
    assert {:ok, plaintext} == PGP.decrypt(private_key, encrypted)
  end
end
