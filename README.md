# Grammar

<pre>
&ltliteral&gt     := number | string | true | false  
&ltexpression&gt  := &ltliteral&gt | &ltidentifier&gt
&ltassignment&gt  := &lttype&gt &ltidentifier&gt = &ltexpression&gt
&ltreturn&gt      := return &ltexpr&gt
&ltinstruction&gt := &ltassignment&gt | &ltreturn&gt
&lttype&gt        := int  
               | char  
               | float  
               | double  
               | void  
               
&ltfunction&gt    := type identifier(void) { instruction* }  
&ltprogram&gt     := &ltfunction&gt*
  
</pre>
