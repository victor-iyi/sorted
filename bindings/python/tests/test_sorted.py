# Copyright 2021 Victor I. Afolabi
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import pytest

from sorted import add_5, greet


@pytest.mark.parametrize(
  ("value", "expected"),
  (
    (0, 5),
    (5, 10),
    (2, 7),
    (-5, 0),
  )
)
def test_add_5(value: int, expected: int) -> None:
    assert add_5(value) == expected


@pytest.mark.parametrize(
  ("name", "expected"),
  (
   ("John", "Hello, John!"),
   ("Jane", "Hello, Jane!"),
   ("Jimmy", "Hello, Jimmy!"),
   ("Smith", "Hello, Smith!"),
   ("John Doe", "Hello, John Doe!"),
  )
)
def test_greet(name: str, expected: str) -> None:
    assert greet(name) == expected
