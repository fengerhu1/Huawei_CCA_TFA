extern void  c_add();
extern unsigned long smc_realm_create(unsigned long rd_addr, unsigned long rlm_para_addr);

int main() {
    int ret_val = 0;
    ret_val = (int) smc_realm_create(0, 0);
    return ret_val;
}