#include <stdio.h>
#include <keystone/keystone.h>

int main(int argc, const char* argv[]) {
	ks_engine *ks;
	ks_err err;
	int asm_err;
	size_t count;
	unsigned char *encode;
	size_t size;

	do
	{
		err = ks_open(KS_ARCH_X86, KS_MODE_64, &ks);
	} while (err != KS_ERR_OK);

	do
	{
		asm_err = ks_asm(ks, argv[1], 0, &encode, &size, &count);
	} while (asm_err != KS_ERR_OK);

	ks_free(encode);

	ks_close(ks);

	for (int i = 0; i < size; i++) 
	{
		printf("%x", encode[i]);
	}

	return 1;
}