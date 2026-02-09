COUNT=5000000000
OUTPUT=/tmp/primes.txt

run:
	./prime_generator.py --progress --parallel $(COUNT) > $(OUTPUT)

activate:
	source venv/bin/activate

requirements:
	pip install -r requirements.txt

init:
	python3 -m venv venv

