/*
 * @Author: your name
 * @Date: 2021-02-22 06:23:36
 * @LastEditTime: 2021-05-09 15:32:51
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: \MDK-ARMf:\project\myRTOS\nucleo-64\hello\RTOS\Src\cpuport.c
 */
#include "osdef.h"

// #define SCB_ICSR    (*((volatile uint32_t *) 0xe000ED04 ))
// #define PENDSV_SET  (1UL << 28UL)
// #define SYSTICK_CLR (1UL << 25UL)

volatile uint8_t interrupt_flag;
volatile uint32_t priority_mask;
uint32_t *next_thread_sp;
uint32_t *current_thread_sp;


/**
  * @brief This function handles Pendable request for system service.
  */
__asm void PendSV_Handler(void)
{
    IMPORT next_thread_sp
    IMPORT current_thread_sp
    IMPORT interrupt_flag
    
    MOVS    R0, #1                  // disable interrupt
    MSR     PRIMASK, R0                

    LDR     R0, =interrupt_flag
    LDR     R0, [R0]
    CBZ     R0, interrupt_switch    // check interrupt flag

    MRS		R0, PSP		            // store PSP to R0
    STMDB 	R0!, {R4 - R11}	        // store R4-R11 
    LDR		R1, =current_thread_sp  // load current_thread psp addr    
    LDR     R1, [R1]          	
    STR		R0, [R1]                // store regesiter info to current_thread's PSP 		

interrupt_switch
    LDR		R0, =next_thread_sp	    // load next_thread psp addr 
    LDR		R0, [R0]		
    LDR 	R0, [R0]
    LDMIA	R0!, {R4 - R11}	        // load next_thread regester info 
    MSR		PSP, R0			        // update PSP
        
    MOVS    R0, #0
    MSR     PRIMASK, R0             // enable interrupt

    ORR     LR, LR, #0x04           // ensure return to PSP
    BX		LR				        // return and switch to next_thread
    ALIGN	4
}


void os_thread_start_first_thread(unsigned int *thread_sp)
{
    next_thread_sp = thread_sp;
    interrupt_flag = 0;
    
	/* clear systick before trigger PendSV, otherwise SysTick_Handler 
     * will be excuted immidiately When enable interrupts,  
     * But the OS hasn't started yet. */
    // enbale SysTick interrupt
    SysTick->CTRL |= SysTick_CTRL_ENABLE_Msk;
    // clear SYSTick mask
    SCB->ICSR |= SCB_ICSR_PENDSTCLR_Msk;    
	// trigger PendSV 
    SCB->ICSR |= SCB_ICSR_PENDSVSET_Msk;	
}


void os_thread_switch(uint32_t *current, uint32_t *next)
{
    interrupt_flag = 1;
    
    current_thread_sp = current;
    next_thread_sp = next;

	// trigger PendSV 
    SCB->ICSR |= SCB_ICSR_PENDSVSET_Msk;	

}


uint32_t os_SysTick_Config(uint32_t ticks)
{
    if ((ticks - 1UL) > SysTick_LOAD_RELOAD_Msk)
    {
        return (1UL);                                     /* Reload value impossible */
    }

    SysTick->LOAD  = (uint32_t)(ticks - 1UL);           /* set reload register */
    NVIC_SetPriority (SysTick_IRQn, 0x00);              /* set Priority for Systick Interrupt */
    SysTick->VAL   = 0UL;                               /* Load the SysTick Counter Value */
    SysTick->CTRL  = SysTick_CTRL_CLKSOURCE_Msk |
                     SysTick_CTRL_TICKINT_Msk   |
                     SysTick_CTRL_ENABLE_Msk;           /* Enable SysTick IRQ and SysTick Timer */
    return (0UL);                                       /* Function successful */
}



