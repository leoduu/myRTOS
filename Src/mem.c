/*
 * @Author: your name
 * @Date: 2021-04-21 10:17:50
 * @LastEditTime: 2021-05-03 19:09:13
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMd:\project\myRTOS\nucleo-64\hello\RTOS\Src\mem.c
 */
#include "mem.h"

#define MEM_SIZE 1024 * 4   // 4k Bytes

static uint8_t os_mem[MEM_SIZE];        
static os_mem_t *mem_begin;    

const uint32_t os_mem_size = sizeof(os_mem_t);

void os_mem_init(void)
{
    // 8 对齐
    memset(os_mem, 0, MEM_SIZE);
    
    mem_begin = (os_mem_t *)os_mem;

    os_mem_t * mem_end;
    mem_end = (os_mem_t *)((uint32_t)os_mem + MEM_SIZE - os_mem_size);

    mem_begin->magic = MEM_FREE;
    mem_begin->prev = NULL;
    mem_begin->next = mem_end;
    
    mem_end->magic = MEM_END;
    mem_end->prev = mem_begin;
    mem_end->next = NULL;    

}

void *os_malloc(const uint32_t size)
{
    os_mem_t *mem = mem_begin;
    uint32_t _size;

    __disable_irq();

    // traverse 
    while (mem->magic != MEM_END) {
        // find a free sapce
        if (mem->magic == MEM_FREE) {

            // check if the spcse is enough
            _size = (uint32_t)mem->next - (uint32_t)mem;
            if (_size >= size) {
                // find a appropriate space
                mem->magic = MEM_USED;     

                // check if the remaining space has at least 1 Byte           
                if (_size > size+os_mem_size) {

                    // make a new mem_node for this apace
                    os_mem_t *mem_new = (os_mem_t *)((uint32_t)mem + os_mem_size+size);
                    mem_new->magic = MEM_FREE;
                    // link these node
                    mem_new->prev = mem;
                    mem_new->next = mem->next;                    
                    mem->next->prev = mem_new;
                    mem->next = mem_new;
                }                
            }
            
            memset((void *)((uint32_t)mem + os_mem_size), 0, size);

            __enable_irq();
            return (void *)((uint32_t)mem + os_mem_size);
        }
        // find next space
        mem = mem->next;
    }

    __enable_irq();
    return NULL;
}


void os_free(void *p)
{
    os_mem_t *mem;
    os_mem_t *mem_prev;
    os_mem_t *mem_next;
    uint8_t flag = 0;   //0:none 1:prev 2:next 3:both

    __disable_irq();

    printf("0x%x\n", (uint32_t)p);
    os_mem_show();

    // get head of this memory
    mem = (os_mem_t *)((uint32_t)p - os_mem_size);
    os_assert(p);
    os_assert((mem->magic&0xFFF0) == 0xABC0);
    mem_prev = mem->prev;
    mem_next = mem->next;

    // merge previous memory 
    if (mem_prev != NULL && mem_prev->magic == MEM_FREE) {
        flag |= 1;      
    }
    // merge next memory 
    if (mem_next != NULL && mem_next->magic == MEM_FREE) {
        flag |= 2;       
    }

    switch (flag)
    {
    case 0:     // none
        mem->magic = MEM_FREE;
        break;
    case 1:     // previous
        mem_next->prev = mem_prev;
        mem_prev->next = mem_next;
        break;
    case 2:     // next
        mem_next->next->prev = mem;
        mem->next = mem_next->next;
        mem->magic = MEM_FREE;
        break;
    case 3:     // both
        mem_prev->next = mem_next->next;
        mem_next->next->prev = mem_prev;
        break;    
    default:
        mem->magic = MEM_FREE;
        break;
    }
    
    os_mem_show();

    __enable_irq();

}

void os_mem_show(void)
{
    os_mem_t *mem = mem_begin;

    printf("\n ___memory map___total:%d______\n|\n",MEM_SIZE);
    //printf("| 0x%x size\n", (mem->magic&0xFFF0)?"USED":"FREE");

    // traverse 
    while (mem->magic != MEM_END) {
        
        printf("|  0x%x  %s  size:%d\n", (uint32_t)mem+os_mem_size, 
               (mem->magic&0x000F)?"USED":"FREE", (uint32_t)mem->next-(uint32_t)mem-os_mem_size);

        // find next space
        mem = mem->next;
    }

    printf("|  0x%x  END\n|_______________________________\n\n", 
           (uint32_t)mem + os_mem_size);
}
